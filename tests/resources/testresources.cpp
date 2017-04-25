/*
 * Copyright (C) 2016 Lucien XU <sfietkonstantin@free.fr>
 *
 * You may use this file under the terms of the BSD license as follows:
 *
 * "Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the
 *     distribution.
 *   * The names of its contributors may not be used to endorse or promote
 *     products derived from this software without specific prior written
 *     permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."
 */

#include "testresources.h"
#include <QMetaMethod>
#include <QResource>
#include <iostream>

void init_testresources()
{
    Q_INIT_RESOURCE(resources);
}

bool invoke_slot(void *ptr)
{
    std::cout << "[C++] Invoking slot for " << ptr << std::endl;
    auto qobject = static_cast<QObject *>(ptr);
    auto metaObject = qobject->metaObject();
    int methodIndex = metaObject->indexOfMethod("test_slot(int)");
    if (methodIndex == -1) {
        std::cout << "[C++] Slot not found" << std::endl;
        return false;
    }
    auto metaMethod = metaObject->method(methodIndex);
    int returned = 0;
    if (!metaMethod.invoke(qobject, Q_RETURN_ARG(int, returned), Q_ARG(int, 42))) {
        std::cout << "[C++] Failed to invoke the slot" << std::endl;
        return false;
    }

    std::cout << "[C++] Received result: " << returned << std::endl;
    return returned == 42;
}

void set_value(void *ptr, int value)
{
    auto qobject = static_cast<QObject *>(ptr);
    auto metaObject = qobject->metaObject();
    int methodIndex = metaObject->indexOfMethod("setValue(int)");
    if (methodIndex == -1) {
        return;
    }

    auto metaMethod = metaObject->method(methodIndex);
    metaMethod.invoke(qobject, Q_ARG(int, value));
}

class ValueChangedSpy : public QObject
{
    Q_OBJECT
public:
    ValueChangedSpy(QObject *parent = nullptr)
        : QObject{parent}
    {
    }
    int value() const
    {
        return m_value;
    }
public slots:
    void slotValueChanged(int value)
    {
        m_value = value;
    }

private:
    int m_value{0};
};

void *create_value_changed_spy(void *ptr)
{
    auto qobject = static_cast<QObject *>(ptr);
    auto spy = new ValueChangedSpy{};
    QObject::connect(qobject, SIGNAL(valueChanged(int)), spy, SLOT(slotValueChanged(int)));
    return spy;
}

void delete_value_changed_spy(const void *ptr)
{
    delete static_cast<const ValueChangedSpy *>(ptr);
}

int value_changed_spy_get_value(const void *ptr)
{
    auto spy = static_cast<const ValueChangedSpy *>(ptr);
    return spy->value();
}

#include "testresources.moc"
