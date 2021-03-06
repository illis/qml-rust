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
#include <QtQml/qqml.h>
#include <iostream>

class Debugger : public QObject
{
    Q_OBJECT
public:
    explicit Debugger(QObject *parent = nullptr)
        : QObject{parent}
    {
    }
public slots:
    void debugObjectMethods(QObject *object)
    {
        auto metaObject = object->metaObject();

        for (int i = 0; i < metaObject->methodCount(); ++i) {
            auto method = metaObject->method(i);
            std::cout << i << ": " << method.methodSignature().data() << " " << methodType(method.methodType())
                      << std::endl;
        }
    }

private:
    static const char *methodType(QMetaMethod::MethodType type)
    {
        switch (type) {
        case QMetaMethod::Method:
            return "method";
        case QMetaMethod::Signal:
            return "signal";
        case QMetaMethod::Slot:
            return "slot";
        case QMetaMethod::Constructor:
            return "constructor";
        default:
            return "<unknown>";
        }
    }
};

void init_testresources()
{
    qmlRegisterSingletonType<Debugger>("debug", 1, 0, "Debugger",
                                       [](QQmlEngine *, QJSEngine *) -> QObject * { return new Debugger{}; });
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

static bool checkProperty(const QMetaObject *metaObject, const char *name, bool writable, bool constant)
{
    int propertyIndex = metaObject->indexOfProperty(name);
    if (propertyIndex == -1) {
        std::cout << "[C++] Property " << name << " not found" << std::endl;
        return false;
    } else {
        const QMetaProperty &property = metaObject->property(propertyIndex);
        bool propertyReadable = property.isReadable();
        bool propertyWritable = property.isWritable();
        bool propertyConstant = property.isConstant();
        std::cout << "[C++] Property " << name << " found: " << propertyIndex << std::endl
                  << "[C++] With readable: " << propertyReadable << ", writable: " << propertyWritable
                  << ", constant: " << propertyConstant << std::endl;
        if (propertyReadable && propertyWritable == writable && propertyConstant == constant) {
            return true;
        } else {
            std::cout << "[C++] Property " << name << " do not satisfy property definition" << std::endl;
            return false;
        }
    }
}

static bool do_check_metatype(const QObject *qobject)
{
    auto metaObject = qobject->metaObject();
    {
        int slotIndex = metaObject->indexOfSlot("set_value(int)");
        if (slotIndex == -1) {
            std::cout << "[C++] Slot set_value not found" << std::endl;
            return false;
        } else {
            std::cout << "[C++] Slot set_value found: " << slotIndex << std::endl;
        }
    }
    {
        int slotIndex = metaObject->indexOfMethod("value()");
        if (slotIndex == -1) {
            std::cout << "[C++] Method value not found" << std::endl;
            return false;
        } else {
            std::cout << "[C++] Method value found: " << slotIndex << std::endl;
        }
    }
    {
        int signalIndex = metaObject->indexOfSignal("value_changed(int)");
        if (signalIndex == -1) {
            std::cout << "[C++] Signal value_changed not found" << std::endl;
            return false;
        } else {
            std::cout << "[C++] Signal value_changed found: " << signalIndex << std::endl;
        }
    }
    if (!checkProperty(metaObject, "value", false, true)) {
        return false;
    }
    if (!checkProperty(metaObject, "value2", false, false)) {
        return false;
    }
    if (!checkProperty(metaObject, "value3", true, false)) {
        return false;
    }
    return true;
}

bool check_metatype(const void *ptr)
{
    auto qobject = static_cast<const QObject *>(ptr);
    return do_check_metatype(qobject);
}

bool check_metatype_qvariant(const void *ptr)
{
    auto variant = static_cast<const QVariant *>(ptr);
    std::cout << "[C++] stored type: " << variant->userType() << std::endl;
    std::cout << "[C++] stored type name: " << QMetaType::typeName(variant->userType()) << std::endl;
    if (!variant->canConvert(QMetaType::QObjectStar)) {
        return false;
    }

    QVariant newVariant{*variant};
    if (!newVariant.convert(QMetaType::QObjectStar)) {
        return false;
    }

    QObject *qobject = qvariant_cast<QObject *>(newVariant);
    if (qobject == nullptr) {
        return false;
    }
    return do_check_metatype(qobject);
}

void connect_qobject_ownership(void *ptr)
{
    auto qobject = static_cast<QObject *>(ptr);
    QObject::connect(qobject, SIGNAL(event()), qobject, SLOT(callback()));
}

void invoke_qobject_ownership_slot(void *ptr)
{
    auto qobject = static_cast<QObject *>(ptr);
    auto metaObject = qobject->metaObject();
    {
        int slotIndex = metaObject->indexOfSlot("send()");
        if (slotIndex == -1) {
            std::cout << "[C++] Slot send not found" << std::endl;
            return;
        } else {
            auto slot = metaObject->method(slotIndex);
            slot.invoke(qobject);
        }
    }
}

#include "testresources.moc"
